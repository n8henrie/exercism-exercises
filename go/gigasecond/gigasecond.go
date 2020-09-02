// This package does something regarding a gigasecond
package gigasecond

import "time"

// Adds a Gigasecond
func AddGigasecond(t time.Time) time.Time {
	return t.Add(1000000000 * time.Second)
}
