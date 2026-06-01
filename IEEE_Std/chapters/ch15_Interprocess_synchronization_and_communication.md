# Chapter 15: Interprocess synchronization and communication

## 15. Interprocess synchronization and communication
### 15.1 General
This clause describes the following:
— Semaphores
— Mailboxes
— Named events
### 15.2 Overview
High-level and easy-to-use synchronization and communication mechanisms are essential to control the
kinds of interactions that occur between dynamic processes used to model a complex system or a highly
reactive testbench.
The basic synchronization mechanism is the named event type, along with the event trigger and event
control constructs (i.e., -> and @). This type of control is limited to static objects. It is adequate for
synchronization at the hardware level and simple system level, but falls short of the needs of a highly
dynamic, reactive testbench.
SystemVerilog also provides a powerful and easy-to-use set of synchronization and communication
mechanisms that can be created and reclaimed dynamically. This set comprises a semaphore built-in class,
which can be used for synchronization and mutual exclusion to shared resources, and a mailbox built-in
class, which can be used as a communication channel between processes.
Semaphores and mailboxes are built-in types; nonetheless, they are classes and can be used as base classes
for deriving additional higher level classes. These built-in classes reside in the built-in std package (see
26.7); thus, they can be redefined by user code in any other scope.
### 15.3 Semaphores
Conceptually, a semaphore is a bucket. When a semaphore is allocated, a bucket that contains a fixed
number of keys is created. Processes using semaphores shall first procure a key from the bucket before they
can continue to execute. If a specific process requires a key, only a fixed number of occurrences of that
process can be in progress simultaneously. All others shall wait until a sufficient number of keys is returned
to the bucket. Semaphores are typically used for mutual exclusion, access control to shared resources, and
basic synchronization.
An example of creating a semaphore is as follows:
semaphore smTx;
semaphore is a built-in class that provides the following methods:
— Create a semaphore with a specified number of keys: new()
— Obtain one or more keys from the bucket: get()
— Return one or more keys into the bucket: put()
— Try to obtain one or more keys without blocking: try_get()

<!-- Page 374 -->

IEEE Std #### 15.3.1 New()
Semaphores are created with the new() method.
The prototype for new() is as follows:
function new(int keyCount = 0);
The keyCount specifies the number of keys initially allocated to the semaphore bucket. The number of keys
in the bucket can increase beyond keyCount when more keys are put into the semaphore than are removed.
This initial value may be negative, but the number of available keys shall be positive before get() or
try_get() can procure keys. The default value for keyCount is 0.
The new() function returns the semaphore handle.
#### 15.3.2 Put()
The semaphore put() method is used to return keys to a semaphore.
The prototype for put() is as follows:
function void put(int keyCount = 1);
The keyCount specifies the number of keys being returned to the semaphore. The default is 1. A negative
value shall result in an error.
When the semaphore.put() function is called, the specified number of keys is returned to the semaphore.
If a process has been suspended waiting for a key, that process shall execute if enough keys have been
returned.
#### 15.3.3 Get()
The semaphore get() method is used to obtain a specified number of keys from a semaphore.
The prototype for get() is as follows:
task get(int keyCount = 1);
The keyCount specifies the required number of keys to obtain from the semaphore. The default is 1. A
negative value shall result in an error.
If the specified number of required keys is less than or equal to the number of available keys, the number of
available keys is reduced by the specified keyCount, the method returns, and execution continues. If the
specified number of keys is not available, the process blocks until the keys become available.
The semaphore waiting queue is first-in first-out (FIFO). This does not guarantee the order in which
processes arrive at the queue, only that their arrival order shall be preserved by the semaphore.
#### 15.3.4 Try_get()
The semaphore try_get() method is used to obtain a specified number of keys from a semaphore, but
without blocking.
The prototype for try_get() is as follows:

<!-- Page 375 -->

IEEE Std function int try_get(int keyCount = 1);
The keyCount specifies the required number of keys to obtain from the semaphore. The default is 1.
If the specified number of required keys is less than or equal to the number of available keys, the number of
available keys is reduced by the specified keyCount, the method returns a positive integer, and execution
continues. If the specified number of keys is not available, the method returns 0. If the requested keyCount
is negative, the method returns 0 and shall result in an error.
### 15.4 Mailboxes
A mailbox is a communication mechanism that allows messages to be exchanged between processes. Data
can be sent to a mailbox by one process and retrieved by another.
Conceptually, mailboxes behave like real mailboxes. When a letter is delivered and put into the mailbox, a
person can retrieve the letter (and any data stored within). However, if the letter has not been delivered when
the mailbox is checked, the person chooses whether to wait for the letter or to retrieve the letter on a
subsequent trip to the mailbox. Similarly, SystemVerilog’s mailboxes provide processes to transfer and
retrieve data in a controlled manner. Mailboxes are created as having either a bounded or unbounded queue
size. A bounded mailbox becomes full when it contains the bounded number of messages. A process that
attempts to place a message into a full mailbox shall be suspended until enough room becomes available in
the mailbox queue. Unbounded mailboxes never suspend a thread in a send operation.
An example of creating a mailbox is as follows:
mailbox mbxRcv;
mailbox is a built-in class that provides the following methods:
— Create a mailbox: new()
— Place a message in a mailbox: put()
— Try to place a message in a mailbox without blocking: try_put()
— Retrieve a message from a mailbox: get() or peek()
— Try to retrieve a message from a mailbox without blocking: try_get() or try_peek()
— Retrieve the number of messages in the mailbox: num()
#### 15.4.1 New()
Mailboxes are created with the new() method.
The prototype for mailbox new() is as follows:
function new(int bound = 0);
The new() function returns the mailbox handle. If the bound argument is 0, then the mailbox is unbounded
(the default) and a put() operation shall never block. If bound is nonzero, it represents the size of the
mailbox queue.
The bound shall be positive. Negative bounds are illegal and can result in indeterminate behavior, but
implementations can issue a warning.

<!-- Page 376 -->

IEEE Std #### 15.4.2 Num()
The number of messages in a mailbox can be obtained via the num() method.
The prototype for num() is as follows:
function int num();
The num() method returns the number of messages currently in the mailbox.
The returned value should be used with care because it is valid only until the next get() or put() is
executed on the mailbox. These mailbox operations can be from different processes from the one executing
the num() method. Therefore, the validity of the returned value depends on the time that the other methods
start and finish.
#### 15.4.3 Put()
The put() method places a message in a mailbox.
The prototype for put() is as follows:
task put(data_type message);
The message canbe any singular expression, including object handles.
The put() method stores a message in the mailbox in strict FIFO order. If the mailbox was created with a
bounded queue, the process shall be suspended until there is enough room in the queue.
#### 15.4.4 Try_put()
The try_put() method attempts to place a message in a mailbox.
The prototype for try_put() is as follows:
function int try_put(data_type message);
The message canbe any singular expression, including object handles.
The try_put() method stores a message in the mailbox in strict FIFO order. If the mailbox is not full, then
the specified message is placed in the mailbox, and the function returns a positive integer. If the mailbox is
full, the method returns 0. Note that unbounded mailboxes are never full.
#### 15.4.5 Get()
The get() method retrieves a message from a mailbox.
The prototype for get() is as follows:
task get(ref data_type message);
The message can be any singular expression, and it shall be a valid left-hand expression.
The get() method retrieves one message from the mailbox, that is, removes one message from the mailbox
queue. If the mailbox is empty, then the current process blocks until a message is placed in the mailbox. If

<!-- Page 377 -->

IEEE Std the type of the message variable is not equivalent to the type of the message in the mailbox, a run-time error
is generated.
Nonparameterized mailboxes are typeless (see 15.4.9), that is, a single mailbox can send and receive
different types of data. Thus, in addition to the data being sent (i.e., the message queue), a mailbox
implementation shall maintain the message data type placed by put(). This is required in order to enable
the run-time type checking.
The mailbox waiting queue is FIFO. This does not guarantee the order in which processes arrive at the
queue, only that their arrival order shall be preserved by the mailbox.
#### 15.4.6 Try_get()
The try_get() method attempts to retrieves a message from a mailbox without blocking.
The prototype for try_get() is as follows:
function int try_get(ref data_type message);
The message can be any singular expression, and it shall be a valid left-hand expression.
The try_get() method tries to retrieve one message from the mailbox. If the mailbox is empty, then the
method returns 0. If the type of the message variable is not equivalent to the type of the message in the
mailbox, the method returns a negative integer. If a message is available and the message type is equivalent
to the type of the message variable, the message is retrieved, and the method returns a positive integer.
#### 15.4.7 Peek()
The peek() method copies a message from a mailbox without removing the message from the queue.
The prototype for peek() is as follows:
task peek(ref data_type message);
The message can be any singular expression, and it shall be a valid left-hand expression.
The peek() method copies one message from the mailbox without removing the message from the mailbox
queue. If the mailbox is empty, then the current process blocks until a message is placed in the mailbox. If
the type of the message variable is not equivalent to the type of the message in the mailbox, a run-time error
is generated.
Calling the peek() method can also cause one message to unblock more than one process. As long as a
message remains in the mailbox queue, any process blocked in either a peek() or get() operation shall
become unblocked.
#### 15.4.8 Try_peek()
The try_peek() method attempts to copy a message from a mailbox without blocking.
The prototype for try_peek() is as follows:
function int try_peek(ref data_type message);
The message can be any singular expression, and it shall be a valid left-hand expression.

<!-- Page 378 -->

IEEE Std The try_peek() method tries to copy one message from the mailbox without removing the message from
the mailbox queue. If the mailbox is empty, then the method returns 0. If the type of the message variable is
not equivalent to the type of the message in the mailbox, the method returns a negative integer. If a message
is available and its type is equivalent to the type of the message variable, the message is copied, and the
method returns a positive integer.
#### 15.4.9 Parameterized mailboxes
The default mailbox is typeless, that is, a single mailbox can send and receive any type of data. This is a very
powerful mechanism, which, unfortunately, can also result in run-time errors due to type mismatches (types
not equivalent) between a message and the type of the variable used to retrieve the message. Frequently, a
mailbox is used to transfer a particular message type, and, in that case, it is useful to detect type mismatches
at compile time.
Parameterized mailboxes use the same parameter mechanism as parameterized classes (see 8.25), modules,
and interfaces:
mailbox #(type = dynamic_type)
where dynamic_type represents a special type that enables run-time type checking (the default).
A parameterized mailbox of a specific type is declared by specifying the type:
typedef mailbox #(string) s_mbox;
s_mbox sm = new;
string s;
sm.put("hello");
...
sm.get(s); // s <- "hello"
Parameterized mailboxes provide all the same standard methods as dynamic mailboxes: num(), new(),
get(), peek(), put(), try_get(), try_peek(), try_put().
The only difference between a generic (dynamic) mailbox and a parameterized mailbox is that for a
parameterized mailbox, the compiler verifies that the calls to put, try_put, peek, try_peek, get, and
try_get methods use argument types equivalent to the mailbox type so that all type mismatches are caught
by the compiler and not at run time.
### 15.5 Named events
An identifier declared as an event data type is called a named event. A named event can be triggered
explicitly. It can be used in an event expression to control the execution of procedural statements in the same
manner as event controls described in 9.4.2. A named event may also be used as a handle assigned from
another named event.
A named event provides a handle to an underlying synchronization object. When a process waits for an
event to be triggered, the process is put on a queue maintained within the synchronization object. Processes
can wait for a named event to be triggered either via the @ operator or by the use of the wait() construct to
examine their triggered state.

<!-- Page 379 -->

IEEE Std #### 15.5.1 Triggering an event
An event is made to occur by the activation of an event triggering statement with the syntax given in
Syntax15-1.
event_trigger ::= // from A.6.5
-> hierarchical_event_identifier nonrange_select ;
| ->> [ delay_or_event_control ] hierarchical_event_identifier nonrange_select ;
Syntax15-1—Event trigger syntax (excerpt from AnnexA)
An event is not made to occur by changing the index of a named event array in an event control expression.
Named events triggered via the -> operator unblock all processes currently waiting on that event. When
triggered, named events behave like a one shot, i.e., the trigger state itself is not observable, only its effect.
This is similar to the way in which an edge can trigger a flip-flop, but the state of the edge cannot be
ascertained, i.e., if (posedge clock) is illegal.
Nonblocking events are triggered using the ->> operator. The effect of the ->> operator is that the statement
executes without blocking, and it creates a nonblocking assign update event in the time in which the delay
control expires or the event control occurs. The effect of this update event shall be to trigger the referenced
event in the nonblocking assignment region of the simulation cycle.
#### 15.5.2 Waiting for an event
The basic mechanism to wait for an event to be triggered is via the event control operator, @.
@ hierarchical_event_identifier;
The @ operator blocks the calling process until the given event is triggered.
For a trigger to unblock a process waiting on an event, the waiting process shall execute the @ statement
before the triggering process executes the trigger operator, ->. If the trigger executes first, then the waiting
process remains blocked.
#### 15.5.3 Persistent trigger: triggered built-in method
SystemVerilog can distinguish the event trigger itself, which is instantaneous, from the named event’s
triggered state, which persists throughout the time step (i.e., until simulation time advances). The
triggered built-in method of a named event allows users to examine this state.
The prototype for the triggered() method is as follows:
function bit triggered();
The triggered method evaluates to true (1'b1) if the given event has been triggered in the current time
step and false (1'b0) otherwise. If the named event is null, then the triggered method returns false.
The triggered method is most useful when used in the context of a wait construct:
wait ( hierarchical_event_identifier.triggered )
Using this mechanism, an event trigger shall unblock the waiting process whether the wait executes before
or at the same simulation time as the trigger operation. The triggered method, thus, helps eliminate a

<!-- Page 380 -->

IEEE Std common race condition that occurs when both the trigger and the wait happen at the same time. A process
that blocks waiting for an event might or might not unblock, depending on the execution order of the waiting
and triggering processes. However, a process that waits on the triggered state always unblocks, regardless of
the order of execution of the wait and trigger operations.
Example:
event done, blast; // declare two new events
event done_too = done; // declare done_too as alias to done
task trigger( event ev );
-> ev;
endtask
...
fork
@ done_too; // wait for done through done_too
#1 trigger( done ); // trigger done through task trigger
join
fork
-> blast;
wait ( blast.triggered );
join
The first fork in the example shows how two event identifiers, done and done_too, refer to the same
synchronization object and also how an event can be passed to a generic task that triggers the event. In the
example, one process waits for the event via done_too, while the actual triggering is done via the trigger
task that is passed done as an argument.
In the second fork, one process can trigger the event blast before the other process (if the processes in the
fork-join execute in source order) has a chance to execute, and wait for the event. Nonetheless, the second
process unblocks and the fork terminates. This is because the process waits for the event’s triggered state,
which remains in its triggered state for the duration of the time step.
An event expression or wait condition is only reevaluated on a change to an operand in the expression, such
as the event prefix of the triggered method. This means that the change of the return value of the
triggered method from 1'b1 to 1'b0 at the end of the current time step will not affect an event control or
wait statement waiting on the triggered method.
#### 15.5.4 Event sequencing: wait_order()
The wait_order construct suspends the calling process until all of the specified events are triggered in the
given order (left to right) or any of the untriggered events are triggered out of order and thus causes the
operation to fail.
The syntax for the wait_order construct is as follows in Syntax15-2.
wait_statement ::= // from A.6.5
...
| wait_order ( hierarchical_identifier { , hierarchical_identifier } ) action_block
action_block ::=
statement_or_null

<!-- Page 381 -->

IEEE Std | [ statement ] else statement_or_null
Syntax15-2—Wait_order event sequencing syntax (excerpt from AnnexA)
For wait_order to succeed, at any point in the sequence, the subsequent events, which shall all be
untriggered at this point or the sequence would have already failed, shall be triggered in the prescribed order.
Preceding events are not limited to occur only once. In other words, once an event occurs in the prescribed
order, it can be triggered again without causing the construct to fail.
Only the first event in the list can wait for the persistent triggered event.
The action taken when the construct fails depends on whether the optional action_block else clause is
specified. If it is specified, then the statement in the else clause (the fail statement) is executed upon failure
of the construct. If the else clause is not specified, a failure generates a default run-time error by calling
$error (see 20.10).
For example:
wait_order(a, b, c);
suspends the current process until events a, b, and c trigger in the order a –> b –> c. If the events trigger
out of order, a run-time error is generated.
For example:
wait_order(a, b, c) else $display("Error: events out of order");
In this example, the fail statement specifies that, upon failure of the construct, a user message be displayed,
but without an error being generated.
For example:
bit success;
wait_order(a, b, c) success = 1; else success = 0;
In this example, the completion status is stored in the variable success, without an error being generated.
#### 15.5.5 Operations on named event variables
An event is a unique data type with several important properties. Named events can be assigned to one
another. When one event is assigned to another, the synchronization queue of the source event is shared by
both the source and the destination event. In this sense, events act as full-fledged variables and not merely as
labels.
15.5.5.1 Merging events
When one event variable is assigned to another, the two become merged. Thus, executing -> on either event
variable affects processes waiting on either event variable.
For example:
event a, b, c;
a = b;
-> c;

<!-- Page 382 -->

IEEE Std -> a; // also triggers b
-> b; // also triggers a
a = c;
b = a;
-> a; // also triggers b and c
-> b; // also triggers a and c
-> c; // also triggers a and b
When events are merged, the assignment only affects the execution of subsequent event control or wait
operations. If a process is blocked waiting for event1 when another event is assigned to event1, the
currently waiting process shall never unblock. For example:
fork
T1: forever @ E2;
T2: forever @ E1;
T3: begin
E2 = E1;
forever -> E2;
end
join
This example forks off three concurrent processes. Each process starts at the same time. Thus, at the same
time that processes T1 and T2 are blocked, process T3 assigns event E1 to E2. As a result, process T1 shall
never unblock because the event E2 is now E1. To unblock both threads T1 and T2, the merger of E2 and E1
has to take place before the fork.
15.5.5.2 Reclaiming events
When an event variable is assigned the special null value, the association between the event variable and
the underlying synchronization queue is broken. When no event variable is associated with an underlying
synchronization queue, the resources of the queue itself become available for reuse.
Triggering a null event shall have no effect. The outcome of waiting on a null event is undefined, and
implementations can issue a run-time warning.
For example:
event E1 = null;
@ E1; // undefined: might block forever or not at all
wait( E1.triggered ); // undefined
-> E1; // no effect
15.5.5.3 Events comparison
Event variables can be compared against other event variables or the special value null. Only the following
operators are allowed for comparing event variables:
— Equality (==) with another event or with null
— Inequality (!=) with another event or with null
— Case equality (===) with another event or with null (same semantics as ==)
— Case inequality (!==) with another event or with null (same semantics as !=)
— Test for a Boolean value that shall be 0 if the event is null and 1 otherwise

<!-- Page 383 -->

IEEE Std Example:
event E1, E2;
if (E1) // same as if (E1 != null)
E1 = E2;
if (E1 == E2)
$display("E1 and E2 are the same event");

<!-- Page 384 -->

IEEE Std 1800-2023