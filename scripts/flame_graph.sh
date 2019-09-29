sudo dtrace -c './target/debug/examples/skipset_add' -o out.stacks -n 'profile-997 /
execname == "skipset_add"/ { @[ustack()] = count(); }'
stackcollapse.pl out.stacks | flamegraph.pl > pretty-graph.svg
open -n -a "Google Chrome" pretty-graph.svg