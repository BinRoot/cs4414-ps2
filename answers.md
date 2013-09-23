Title: Problem Set 2 Answers
========

Authors: Nishant Shukla, Jasdev Singh
--------

1. **Nishant:**

    According to the man-page, CPU usage is expressed as the % of time spent running during the entire lifetime of a process.
    Ergo ∑CPU(pid) > 100% is possible.

    **Jasdev:**

    I was curious how process IDs were generated, so I did a search on this. Turns out that IDs are generated sequentially (from 0) until a system-dependent maximum value. At this point, the numbers become recycled and regenerated from the beginning. This is the approach most Unix implementations take. Although, more complex schemes have been used [http://www.faqs.org/faqs/aix-faq/part2/section-20.html].

2. **Nishant:**

    Pressing '1' in top shows stats on all four cores. It seems like only one core maximizes the 'us' (user space) value. The other 3 cores probably focus on non-user space tasks (background tasks).

    **Jasdev:**

    Noticed that certain process IDs had a trailing '-'. Turns out this is present for 32-bit architectures. On the other hand a '+' is present for 64-bit architectures. Lastly, a '*' is present for non-native architectures. this can be used to sort processes that utilize your native architecture better than others.

3. **Nishant:**

    This one stores number of last few bash processes running last hour (7pm) to a txt file.
    ps aux | grep -i 19: | tail | grep -i bash | wc -l > num_of_bash_running_recently_last_hour.txt

    **Jasdev:**

    If you ever have a huge amount of text that you need to use in a pipe but still want it to be readable, simply type:
    pbpaste | ...
    This will pipe your clipboard's output into successive commands which can be super useful for kicking off a long string of them.