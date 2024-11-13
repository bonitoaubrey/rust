A VecDeque (pronounced "vec-deck") is a Vec that is optimized for (i.e., good at) popping items both off the front and the back.

It uses something called a ring buffer.

In general, VedDeque is a bit slower than a Vec, but if you have to do things on both ends, it is much faster, thanks to the buffer.

