package letter

// FreqMap records the frequency of each rune in a given text.
type FreqMap map[rune]int

// Frequency counts the frequency of each rune in a given text and returns this
// data as a FreqMap.
func Frequency(s string) FreqMap {
	m := FreqMap{}
	for _, r := range s {
		m[r]++
	}
	return m
}

// ConcurrentFrequency gets letter frequency concurrently
func ConcurrentFrequency(s []string) FreqMap {
	m := FreqMap{}
	ch := make(chan FreqMap, 10)
	for _, str := range s {
		go func(str string) {
			mp := Frequency(str)
			ch <- mp
		}(str)
	}
	for range s {
		for r, c := range <-ch {
			m[r] += c
		}
	}
	return m
}
