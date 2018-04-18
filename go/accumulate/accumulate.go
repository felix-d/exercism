package accumulate

const testVersion = 1

func Accumulate(collection []string, mapper func(string) string) []string {
	for i, x := range collection {
		collection[i] = mapper(x)
	}
	return collection
}
