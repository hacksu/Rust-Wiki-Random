# Rust Wiki Rand

Say that 10 times fast.

Anyway. We'll talking about Rust today. To follow the talk you'll need to have
Rust installed on computer. If you don't already now would be a good time to
install it. Go to https://www.rust-lang.org/ and follow the instructions there.
One that's installed go to https://github.com/hacksu/Rust-Wiki-Random and
download the repository.

If anything confuses you don't hesitate to stop someone and ask them what to do.

How many of you know C++? probably most from taking it at Kent. How many of you
know Python? Which is better? If your like most people you may have just
said Python. Some of you may have said C++. Those people who said C++ probably
had a lot of reasons for it, but one of them probably boils down to that it's
faster and has lower overhead. You may have also thrown something in there about
being "typed". The point is both sets of people are right. Python and C++ are
meant to do very different things. Python is great for making applications and
quick small projects where efficiency isn't key. Sometimes the project your
making is neither quick nur small and efficiency is key. In those cases you need
something else. Basically you need a systems programing language. C++ isn't the
only one of those, but it is one of the most popular and also one of the oldest.
Being old is good in some ways, but in others it's not. The bad is mostly in the
lack of an ability to change. Language design like everything else has gotten
better in the last 50 years. C++ really hasn't much.

So if you sit down with the idea you want a new systems programing language you
have a few options. Go, Rust, and D spring to my mind. Others may think of
something else. Of these Rust seems to the most popular right now so I figured
why not try to give an introduction. The info I gave last week applies here too.
I'm not going to be able to teach everything you need to know to use Rust today.
I'm going to try to keep things even briefer than I did last week because we
ended up not getting through everything last week. This is a really rough into.
I can only suggest if you want to get better at it make a project with it. This
project is the start of one such possible project. If you like it make more with
it.

## Cargo and Hello World

One of the advantages I didn't mention with Rust is it's superior
standardization. I almost feel bad saying this, but the amount of trouble I've
had trying to install relatively simple C++ libraries is incredible. Node has
npm. Python has pip. C++ has nothing. Rust though has Cargo. Cargo is similar to
npm if your familiar with that. It's both a build system and a dependency
system. It Will collect all our dependencies and turn them into one executable.
We can even tell it to run our project and compile at the same time.

To start with just download and extract or clone the repository at
https://github.com/hacksu/Rust-Wiki-Random and cd into it in your terminal. Then
run `cargo run`. It should compile the project and spit out `Hello World!`. If
it doesn't something been installed wrong.

Open the folder in your text editor of choice. I find Atom can be opened with
`atom .` Take a look at `main.rs` (it may help to install language support).

Right now it just has:

        fn main() {
            println!("Hello World!")
        }
Like many languages when the executable is run, the `main` function is called.
In rust functions are defined with the fn keyword. The next line should also
make sense to to people who've programmed in other languages. It's a "function"
call to print the string it's passed (actually a macro, we'll get to that in a
sec).

## Let's do math for fun and... profit?

This is probably the most boring program ever lets do stuff, math to start.
<pre>
        fn main() {
            <b>let i = 19;</b>
            <b>println!("Hello {}!", i);</b>
        }
</pre>

When you run this with `cargo run` you should see `Hello 19!`. The reason I said
`println!` is a macro not a function is visible here. It's actually at compile
time turning `println!("Hello {}!", i);` into something much uglier that happens
to print i just the way we want it to. This also means though that things like
the number of arguments passed can be checked at compile time.

There is isn't much math yet:

<pre>
        fn main() {
            <b>let i = 19;</b>
            <b>i+= 1;</b>
            <b>println!("Hello {}!", i);</b>
        }
</pre>

Run it and you get... a compiler error. Why? Well it tells us `error:
re-assignment of immutable variable`. This is a bit strange but in Rust we need
to say if and when we want a variable that can be changed. That's mutable. It's
a small change. Just add `mut` after the `let` like.

<pre>
        fn main() {
            let <b>mut</b> i = 19;
            i+= 1;
            println!("Hello {}!", i);
        }
</pre>

Now when we run it we get `Hello 20!` just like we might expect.
