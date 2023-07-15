## Tokio Tasks

A Tokio task is an asynchronous green thread.

> A green thread, also known as a lightweight thread or cooperative thread, is a thread-like abstraction managed by a runtime or a library rather than the operating system. Unlike native threads that are managed by the operating system's scheduler, green threads are implemented and scheduled at the application level.
> 
> Green threads provide concurrency by allowing multiple execution paths within a single operating system thread. They can be created, scheduled, and synchronized by the runtime or library without relying on the underlying operating system thread model. Green threads typically have lower memory and processing overhead compared to native threads, as they are implemented and managed entirely in user space.
> 
> Green threads are called "cooperative" because their execution relies on cooperative multitasking. It means that a green thread voluntarily yields control to other green threads, typically by explicitly yielding or performing certain blocking operations that allow the scheduler to switch to another green thread. This cooperative nature requires careful programming to ensure that each green thread yields control appropriately, avoiding scenarios where a single misbehaving green thread can monopolize the execution.

They are created by passing an `async` block to `tokio::spawn`. The `tokio::spawn` returns a `JoinHandle.` The Caller may obtain the return value of the `aysnc` function using `.await` on the `JoinHandle.`

```rust
#[tokio::main]
async fn main() {
	let handler = tokio::spawn(async {
		// -- snip --
	});

	let out = handler.await.unwrap(); // Returns Result wrapped returned value
	println!("{:#?}", out);
}
```

> Tasks are unit of execution managed by the scheduler. Spawning the task submits it to the Tokio scheduler. The spawned task may be executed on the same thread as where it was spawned, or it may execute on a different runtime thread. The tasks can also be moved between threads after being spawned.

### Tokio Task Size

Tokio tasks are very lightweight and requires only a single allocation and 64 bytes of memory.

### Tokio Task Rules

- The spawned task must not contain any references to data owned outside the task.

```rust
use tokio::task;

#[tokio::main]
async fn main() {
	let v = vec![1, 2, 3];

	task::spawn(async move { // here we moved all references to the scope of the spawn
		println!("{:?}", v);
	});
}
```

- To share a single piece of data,  it must be shared using synchronization primitives such as `Arc`.

### Send Bound

The task spawned by `tokio::spawn` must implement `Send` to allow Tokio runtime to move the tasks between threads while they are suspended at an `.await`.

Tasks are `Send` when **all** data that is held **across** `.await` calls is `Send`. This is a bit subtle. When `.await` is called, the task yields back to the scheduler. The next time the task is executed, it resumes from the point it last yielded. To make this work, all state that is used **after** `.await` must be saved by the task. If this state is `Send`, i.e. can be moved across threads, then the task itself can be moved across threads. Conversely, if the state is not `Send`, then neither is the task.

```rust
use tokio::task::yeild_now;
use std::rc::Rc; // `Rc` does not implement `Send`

#[tokio::main]
async fn main() {
	tokio::spawn(async {
		// This scope forces `Rc` to drop before `.await`
		{
			let rc = Rc::new("Hello");
			println!("{}", rc);
		}

		// `rc` is no longer used. It is **not** presisted when
		// the task is yields ot the scheduler
		yield_now().await;
	});
}
```

