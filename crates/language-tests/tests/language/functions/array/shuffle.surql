/**
[test]

[[test.results]]
value = "[]"

[[test.results]]
error = "Incorrect arguments for function array::shuffle(). Argument 1 was the wrong type. Expected a array but found 3"

[[test.results]]
value = "[4]"

[[test.results]]
value = "[1, 1, 1]"

[[test.results]]
match = """
	1 in $result && 2 in $result && 3 in $result && 4 in $result && "text" in $result
"""

*/
array::shuffle([]);
array::shuffle(3);
array::shuffle([4]);
array::shuffle([1,1,1]);
array::shuffle([1,2,"text",3,3,4]); // find a way to check randomness
