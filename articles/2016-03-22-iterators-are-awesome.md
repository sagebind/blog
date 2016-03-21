+++
title = "Iterators Are Awesome"
author = "Stephen Coakley"
date = "2016-03-22 America/Chicago"
category = "software-design"
+++

Iterators are awesome. Maybe you've heard this before and already are an iterator master, but for a surprisingly large number of programmers, the term "iterator" is a scary term that reminds you of your confusing Software Engineering class in college. I'm here to illuminate on what iterators are,  why they matter, and how they are used. When wielded properly, they can help you write code that is cleaner, simpler to read, and sometimes even more performant.

First, lets start with the basics of demystifying what iterators are, and then we'll explore their uses and benefits. We will also look at some practical code samples and how to use iterators in different programming languages. Most of the initial code samples will be in PHP because the way PHP defines iterators makes them easier to read, but I will transition to C++ and then other languages as we progress.


## What are iterators?
Iterators are an extension on a simple concept that you're already familiar with. If you've ever done any substantial amount of programming before, chances are that you've looped over an array before. You may have written code like this:

```php
$numbers = [1, 1, 2, 3, 5, 8];
$sum = 0;
for ($i = 0; $i < count($numbers); $i++) {
    $sum += $numbers[$i];
}
```

_The above code is in PHP, but this pattern is nearly identical in C, C++, Java, C#, JavaScript, and many other languages._

Guess what? This is a form of iteration! _Iteration_ simply means to _traverse_ over some sort of collection or sequence of values, until you reach the end of the values. You could iterate over an array, a vector, a sequence of numbers, or rows from a database query.

So now we know what _iteration_ is, but what about _iterators_? An iterator is a specialized object whose sole purpose is to perform iteration on some sequence of data. It is important to note that an iterator is not the data itself; it is a separate object from the data being iterated over. Regardless over what is being iterated, an iterator's job is to keep track of where you are in the list of values, and let you move to the next one, if there is one.


## Why should I care?
If you've been using regular `for` loops for ages and they work fine, why should you care about iterators? I'll give you three reasons:

1. You are bound to encounter them in real code, and then you'll have to understand them.
2. They offer a simpler syntax that works for any type of collection.
3. They provide a better means for looping over a sequence with an unknown number of items.

The first is self-explanatory, the second you will find out in a bit, but the third will need some explanation. Normally when looping over collection, you need to know the range of indexes that you will be looping over, usually from 0 to the end of the collection. You don't always know how many items there will be; in a database query, for example, you have to simply read all the resulting rows until the end to determine how many there are, and by that time, the entire result set is now stored in memory. Linked lists are another example; determining the size can be a costly operation, but iterators inherently don't need to know the size of the collection. Iterators don't offer as much control as manual loops, but they let you save memory by reading a single row at a time, as they arrive from the database.


## Using iterators
How might you use an iterator to loop over an array? Most programming languages worth their salt support iterators, but how they are used usually have slight syntax differences from languages to language. Let's look at another PHP example to parallel the previous code sample:

```php
$numbers = [1, 1, 2, 3, 5, 8];
$sum = 0;
for ($iter = new ArrayIterator($numbers); $iter->valid(); $iter->next()) {
    $sum += $iter->current();
}
```

Whoa, there's a lot going on in there! You might catch the word "iterator" there, but let's break it down and understand what this code is doing. The definition of the array and sum variables are the same, so let's inspect the `for` loop. First, we have our "initialization" statement as

```php
$iter = new ArrayIterator($numbers);
```

This step runs once before we start looping; here we create a new `ArrayIterator` object with our array as the constructor parameter. This `$iter` variable _is_ our iterator object. More on that in a bit. Let's look at our loop condition:

```php
$iter->valid();
```

Valid? This method returns `true` or `false`, depending on if there are still more elements in the array to loop over. When we reach the last element, the iterator is no longer "valid", so the `valid()` method returns `false`. Then we have our "advance" step:

```php
$iter->next()
```

Here the `next()` method moves the iterator to the "next" item in the array, similar to `i++` in a traditional `for` loop. Now for the really curious bit:

```php
$sum += $iter->current();
```

Instead of adding `$numbers[$i]` to the sum, we use the `current()` method of the iterator. You see, the iterator object keeps track of where we are in the list of items in `$numbers` for us. All we have to do is call `current()` to get the element the iterator is currently looking at, and `next()` to move the iterator to the next item.

With some more understanding under our belt, let's compare this with an iterator in C++:

```cpp
vector<int> numbers = {1, 1, 2, 3, 5, 8};
int sum = 0;
for (vector<int>::iterator iter = numbers.begin(); iter != numbers.end(); iter++) {
    sum += *iter;
}
```

Admittedly, this is a bit more cryptic, but we have all the same parts as the previous code. In this case, `numbers.begin()` returns a new iterator object of type `vector<int>::iterator` that you use to iterate the `numbers` vector. `iter != numbers.end()` does the same thing as `valid()`, and `iter++` is just an operator overload to move to the next item. Yet another operator overload (the dereference `*` operator) gives us the current element with `*iter`. You don't have to understand operator overloading to use iterators, but it may help since a _lot_ of C++ standard library stuff uses them (probably a little _too_ much).


### "For each" syntax
Using an iterator definitely requires more code than a simple `for` loop. Since iterators are so common and the syntax for using them is almost always the same every time, most languages provide an alternate type of loop, usually called a "for each" loop, that handles calling the proper iterator methods for you. In PHP, this loop is aptly called a `foreach` loop, and greatly reduces the amount of code:

```php
$numbers = [1, 1, 2, 3, 5, 8];
$sum = 0;
foreach ($numbers as $current) {
    $sum += $current;
}
```

Isn't that so much nicer? The `foreach` loop handles calling the `valid()`, `current()`, and `next()` methods for us, and simply gives us the current value that we want each time the loop goes around. If you're using [C++11](https://en.wikipedia.org/wiki/C%2B%2B11) or newer, you can do the same in C++:

```cpp
vector<int> numbers = {1, 1, 2, 3, 5, 8};
int sum = 0;
for (int current : numbers) {
    sum += current;
}
```


### Comparing iterators across languages
Now that we understand how iterators work and are used, let's compare the syntax for some common programming languages other than C++ and PHP. You will see that most languages have at least two interfaces involving iterators: an interface for array-like types that you can get an iterator _for_, and an interface for the iterator itself. In PHP's case, these interfaces are the aptly-named [`Iterator`](http://php.net/manual/en/class.iterator.php) and [`IteratorAggregate`](http://php.net/manual/en/class.iteratoraggregate.php).


#### Java
Java's iterators aren't as cryptic as C++'s iterators, but are a pain to write your own. Using them isn't so bad at least:

```java
ArrayList<Integer> numbers = new ArrayList<Integer>();
numbers.add(1);
numbers.add(2);
numbers.add(3);

int sum = 0;
for (Integer current : numbers) {
    sum += current;
}
```

The Java "enhanced for loop" has the same syntax as the for each loop in C++. The downside is that you can't use iterators on scalar item types like `int`, or on arrays. Java uses the [`Iterator`](https://docs.oracle.com/javase/8/docs/api/java/util/Iterator.html) and [`Iterable`](https://docs.oracle.com/javase/8/docs/api/java/lang/Iterable.html) interfaces for iterators:

```java
for (Iterator<Integer> iter = numbers.iterator(); iter.hasNext(); )
{
    Integer current = iter.next();
    sum += current;
}
```


#### C#
C# does iteration very well. It is _slightly_ confusing since iterators are called "Enumerators", but the interface is very simple to implement and use:

```csharp
int[] numbers = new int[] {1, 1, 2, 3, 5, 8};
int sum = 0;
foreach (int current in numbers)
{
    sum += current;
}
```

The `foreach` loop is syntax for working with an object implementing the [`IEnumerator`](https://msdn.microsoft.com/en-us/library/78dfe2yb.aspx) or [`IEnumerable`](https://msdn.microsoft.com/en-us/library/9eekhta0.aspx) interfaces:

```csharp
for (IEnumerator<int> enumerator = numbers.GetEnumerator(); enumerator.MoveNext(); )
{
    sum += enumerator.Current;
}
```


#### Python
Python is pretty simple, as usual:

```python
numbers = [1, 1, 2, 3, 5, 8]
sum = 0
for current in numbers:
    sum += current
```

Python doesn't have interfaces, but uses "magic" [`__iter__`](https://docs.python.org/3/reference/datamodel.html#object.__iter__) and [`__next__`](https://docs.python.org/3/reference/datamodel.html#object.__next__) methods, which frankly are really strange and and you should never, ever use them by hand.


#### Rust
Rust is slowly becoming one of my favorite languages, so of course I'll include it here. Rust doesn't even have traditional C-style `for` loops; the `for` loop is designed exclusively for iterators:

```rust
let numbers = vec![1, 1, 2, 3, 5, 8];
let mut sum = 0;
for current in numbers {
    sum += current;
}
```

Iterators in Rust implement the [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) trait, which provides a lot of built-in methods, but we only need the `next()` method to perform iteration:

```rust
let mut iter = numbers.iter();
loop {
    match iter.next() {
        Some(current) => {
            sum += current;
        },
        None => { break }
    }
}
```
