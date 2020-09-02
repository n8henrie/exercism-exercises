// Package bob says stuff in response
package bob

import "strings"

// Hey does some stuff
// // # Bob
//
// Bob is a lackadaisical teenager. In conversation, his responses are very limited.
//
// Bob answers 'Sure.' if you ask him a question.
//
// He answers 'Whoa, chill out!' if you yell at him.
//
// He answers 'Calm down, I know what I'm doing!' if you yell a question at him.
//
// He says 'Fine. Be that way!' if you address him without actually saying
// anything.
//
// He answers 'Whatever.' to anything else.

func isAllUpper(remark string) (allUpper bool) {
	println(remark)
	for _, c := range remark {
		if 'A' <= c && c <= 'Z' {
			allUpper = true
		} else if 'a' <= c && c <= 'z' {
			return false
		}
	}
	return
}

func Hey(remark string) string {
	remark = strings.TrimSpace(remark)
	switch {
	case strings.HasSuffix(remark, "?") && isAllUpper(remark):
		return "Calm down, I know what I'm doing!"
	case strings.HasSuffix(remark, "?"):
		return "Sure."
	case isAllUpper(remark):
		return "Whoa, chill out!"
	case remark == "":
		return "Fine. Be that way!"
	default:
		return "Whatever."
	}
}
