use core::{pin::Pin, task::{Context, Poll}};

use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use futures_util::{stream::Stream, task::AtomicWaker, StreamExt};

use crate::{println, keyboard::handle_key_press};

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();
const QUEUE_CAPACITY: usize = 1024;

pub struct ScancodeStream {
    _private: () // To prevent construction from outside of module
}

impl ScancodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(QUEUE_CAPACITY)).expect("Queue already initialized");
        ScancodeStream { _private: () }
    }

    pub async fn get_next(&mut self) -> u8 {
        self.next().await.unwrap()
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE.try_get().expect("Queue uninitialized");

        if let Some(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(&ctx.waker());

        match queue.pop() {
            None => Poll::Pending,
            Some(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
        }
    }
}

pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            println!("Warning: Scancode queue full, input dropped");
        } else {
            WAKER.wake();
        }
    } else {
        println!("Warning: Scancode queue isn't initialized");
    }
}

pub async fn print_keypress() {
    let mut scancode_stream = ScancodeStream::new();

    loop {
        handle_key_press(&mut scancode_stream).await;
    }
}
