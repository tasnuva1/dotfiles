we wanna return a pointer. instead of returning bar itself, we wanna return the address at which bar is sitting at.
nothing is referencing this thing anymore let's get rid of it, deallowcated.
So, we know excectly, when this will be de-allowcated. statically, it's right here at line 30.
you can have as many reader you want, but not writer - reference. only one writer at a time. cause writing at the same time is not allowed. many reader or one writer reference.
there is only one owner, and owner is responsible deallowcating the memory / charge of that memeory.
So you should always always always use an immutable borrow if you can.
in each loop you are checking...
