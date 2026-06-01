# Annex G: Annex G

## Annex G
(normative)
Std package
G.1 General
This annex describes the contents of the built-in standard package, including the following:
— The semaphore class
— The mailbox class
— The randomize function
— The process class
— The weak reference class
G.2 Overview
The standard package contains system types (see 26.7). The following types are provided by the std built-in
package. The descriptions of the semantics of these types are defined in the indicated subclauses.
G.3 Semaphore
The semaphore class is described in 15.3, and its prototype is as follows:
class semaphore;
function new(int keyCount = 0);
function void put(int keyCount = 1);
task get(int keyCount = 1);
function int try_get(int keyCount = 1);
endclass
G.4 Mailbox
The mailbox class is described in 15.4, and its prototype is as follows:
The dynamic_singular_type below represents a special type that enables run-time type checking.
class mailbox #(type T = dynamic_singular_type) ;
function new(int bound = 0);
function int num();
task put( T message);
function int try_put( T message);
task get( ref T message );
function int try_get( ref T message );
task peek( ref T message );
function int try_peek( ref T message );
endclass

<!-- Page 1259 -->

IEEE Std G.5 Randomize
The randomize function is described in 18.12, and its prototype is as follows:
function int randomize( ... );
The syntax for the randomize function is defined as randomize_call in A.8.2. The specific form applicable to
std::randomize is summarized here:
randomize { attribute_instance } [ ( [ variable_identifier_list ] ) ]
[ with constraint_block ]
G.6 Process
The process class is described in 9.7, and its prototype is as follows:
class :final process;
typedef enum {FINISHED, RUNNING, WAITING, SUSPENDED, KILLED} state;
static function process self();
function state status();
function void kill();
task await();
function void suspend();
function void resume();
function void srandom(int seed);
function string get_randstate();
function void set_randstate(string state);
endclass
G.7 Weak reference
The weak reference class is described in 8.30, and its prototype is as follows:
class weak_reference #(type class T);
function new(T referent);
function T get();
function void clear();
static function longint get_id(T obj);
endclass

<!-- Page 1260 -->

IEEE Std 1800-2023