use core::{task::{Waker, Context, Poll}, mem::size_of};

use alloc::{collections::BTreeMap, sync::Arc, task::Wake};
use crossbeam_queue::ArrayQueue;
use x86_64::instructions::interrupts;

use super::{Task, TaskId};

pub struct Executor {
    tasks: BTreeMap<TaskId, Task>,
    task_queue: Arc<ArrayQueue<TaskId>>,
    waker_cache: BTreeMap<TaskId, Waker>
}

const MAX_PROCESSES: usize = 8192 / size_of::<Executor>();

struct TaskWaker {
    task_id: TaskId,
    task_queue: Arc<ArrayQueue<TaskId>>
}

impl TaskWaker {
    fn new(task_id: TaskId, task_queue: Arc<ArrayQueue<TaskId>>) -> Waker {
        Waker::from(Arc::new(TaskWaker {
            task_id,
            task_queue
        }))
    }

    fn wake_task(&self) {
        self.task_queue.push(self.task_id).expect("Task queue is full");
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.wake_task();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_task();
    }
}

impl Executor {
    pub fn new() -> Executor {
        Executor {
            tasks: BTreeMap::new(),
            task_queue: Arc::new(ArrayQueue::new(MAX_PROCESSES)), // fits in 8 KiB
            waker_cache: BTreeMap::new()
        }
    }

    pub fn spawn(&mut self, task: Task) {
        let task_id = task.id;

        if self.tasks.insert(task_id, task).is_some() {
            panic!("Task already queued");
        }

        self.task_queue.push(task_id).expect("Task queue is fulled")
    }

    fn idle_sleep(&self) {
        interrupts::disable();

        if self.task_queue.is_empty() {
            interrupts::enable_and_hlt();
        } else {
            interrupts::enable();
        }
    }

    pub fn run(&mut self) -> ! {
        loop {
            self.run_ready_tasks();
            self.idle_sleep();
        }
    }

    fn run_ready_tasks(&mut self) {
        let Self {
            tasks,
            task_queue,
            waker_cache
        } = self;

        while let Some(task_id) = task_queue.pop() {
            let task = match tasks.get_mut(&task_id) {
                Some(task) => task,
                None => continue
            };
            let waker = waker_cache.entry(task_id).or_insert_with(|| TaskWaker::new(task_id, task_queue.clone()));
            let mut context = Context::from_waker(waker);
            match task.poll(&mut context) {
                Poll::Ready(()) => {
                    tasks.remove(&task_id);
                    waker_cache.remove(&task_id);
                }
                Poll::Pending => {}
            }
        }
    }
}
