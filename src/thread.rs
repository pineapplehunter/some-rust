use core::{cell::UnsafeCell, marker::PhantomData, ptr::NonNull};

use alloc::{boxed::Box, collections::VecDeque, sync::Arc};

use crate::sync::Mutex;

struct TaskReturnValue {
    handled: bool,
    ptr: Option<NonNull<()>>,
}

unsafe impl Sync for TaskReturnValue {}
unsafe impl Send for TaskReturnValue {}

pub struct Task {
    return_value: Arc<Mutex<TaskReturnValue>>,
    inst: Box<dyn FnOnce(Arc<Mutex<TaskReturnValue>>) + Send + 'static>,
}

// this struct is only used for communication with a single thread
unsafe impl Sync for Task {}
unsafe impl Send for Task {}

impl Task {
    fn run(self) {
        (self.inst)(self.return_value)
    }
}

pub struct ThreadSafeQueue {
    inner: Mutex<VecDeque<Task>>,
}

impl ThreadSafeQueue {
    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(VecDeque::new()),
        }
    }

    pub fn push(&self, task: Task) {
        let mut guard = self.inner.lock();
        guard.push_back(task);
    }

    pub fn pop(&self) -> Option<Task> {
        let mut guard = self.inner.lock();
        guard.pop_front()
    }

    pub fn len(&self) -> usize {
        let guard = self.inner.lock();
        guard.len()
    }

    pub fn is_empty(&self) -> bool {
        let guard = self.inner.lock();
        guard.is_empty()
    }
}

static MULTITHREAD_TASKS: ThreadSafeQueue = ThreadSafeQueue::new();

pub struct TaskHandle<R: Send> {
    return_value: Arc<Mutex<TaskReturnValue>>,
    _return_type: PhantomData<R>,
}

impl<R: Send> Drop for TaskHandle<R> {
    fn drop(&mut self) {
        let mut guard = self.return_value.lock();

        // may not have a value if the result is not set, or join was called
        if let Some(return_value_ptr) = guard.ptr.take() {
            let _ = Box::from(return_value_ptr.as_ptr() as *mut R);
        }

        // tells the task that it is not handled anymore
        guard.handled = false;
    }
}

impl<R: Send> TaskHandle<R> {
    pub fn join(self) -> Box<R> {
        loop {
            let ptr = match self.return_value.lock().ptr.take() {
                Some(ptr) => ptr,
                None => continue,
            };

            // safe because the return data is always type R
            let b = unsafe { Box::from_raw(ptr.as_ptr() as *mut R) };
            return b;
        }
    }

    pub fn is_finished(&self) -> bool {
        self.return_value.lock().ptr.is_some()
    }
}

pub fn spawn<F, T>(thread_fn: F) -> TaskHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send,
{
    // wrap the thread function so it doesn't have the type information
    let inst = Box::new(|ret: Arc<Mutex<TaskReturnValue>>| {
        let r = thread_fn();
        let mut guard = ret.lock();
        // if it is not handled by `TaskHandle` the result value will not be freed
        // we dont create a box with the result until we are sure that it is handled
        if guard.handled {
            // create a pointer to the return value in heap
            // delete the type information so any type can be returned
            let box_ptr = Box::leak(Box::new(r)) as *mut T as *mut ();
            let old = guard.ptr.replace(NonNull::new(box_ptr).unwrap());
            assert_eq!(old, None);
        }
    });

    // create a pointer to a pointer which holds the returned data
    let return_value_ptr = Arc::new(Mutex::new(TaskReturnValue {
        handled: true,
        ptr: None,
    }));

    let task = Task {
        inst,
        return_value: return_value_ptr.clone(),
    };

    // add the task to the queue
    MULTITHREAD_TASKS.push(task);

    TaskHandle {
        return_value: return_value_ptr,
        _return_type: PhantomData,
    }
}

fn fetch_task() -> Option<Task> {
    MULTITHREAD_TASKS.pop()
}

#[inline(never)]
pub fn event_loop() {
    loop {
        let task = match fetch_task() {
            Some(task) => task,
            None => continue,
        };
        task.run()
    }
}

#[inline(never)]
pub fn event_loop_until_empty() {
    loop {
        let Some(task) = fetch_task() else {
            break;
        };
        task.run()
    }
}

struct Block<T> {
    next: UnsafeCell<Option<NonNull<Self>>>,
    data: UnsafeCell<Option<T>>,
}

pub struct Sender<T> {
    block: Mutex<NonNull<Block<T>>>,
}

pub struct Receiver<T> {
    block: Mutex<NonNull<Block<T>>>,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let block = NonNull::new(Box::leak(Box::new(Block {
        next: UnsafeCell::new(None),
        data: UnsafeCell::new(None),
    })) as *mut Block<T>)
    .unwrap();

    let sender = Sender {
        block: Mutex::new(block),
    };

    let receiver = Receiver {
        block: Mutex::new(block),
    };

    (sender, receiver)
}

impl<T> Sender<T> {
    pub fn send(&self, data: T) {
        let next_block = NonNull::new(Box::leak(Box::new(Block {
            next: UnsafeCell::new(None),
            data: UnsafeCell::new(Some(data)),
        })) as *mut Block<T>)
        .unwrap();

        let mut guard = self.block.lock();
        unsafe {
            *(*guard.as_ptr()).next.get_mut() = Some(next_block);
            *guard = next_block;
        };
    }
}

impl<T> Receiver<T> {
    pub fn try_recv(&self) -> Option<T> {
        let mut guard = self.block.lock();

        loop {
            if let Some(data) = unsafe { (*guard.as_ptr()).data.get_mut().take() } {
                return Some(data);
            }
            if let Some(next) = unsafe { (*guard.as_ptr()).next.get_mut().take() } {
                let this_ptr = *guard;
                *guard = next;
                // free memory
                let _ = unsafe { Box::from_raw(this_ptr.as_ptr()) };
                continue;
            }
            return None;
        }
    }

    pub fn recv(&self) -> T {
        loop {
            if let Some(data) = self.try_recv() {
                return data;
            }
        }
    }
}
