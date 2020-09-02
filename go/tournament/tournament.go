// Package tournament helps parse some tournament data
package tournament

import (
	"bufio"
	"fmt"
	"io"
	"sort"
	"strings"
)

func parseLine(line string, tally map[string]team) error {
	if line == "" || strings.HasPrefix(line, "#") {
		return nil
	}
	parts := strings.Split(line, ";")
	if len(parts) != 3 {
		return fmt.Errorf("bad match format %q: expected 'teamA;teamB;result'", line)
	}
	t1Name := parts[0]
	t2Name := parts[1]
	outcome := parts[2]

	t1 := tally[t1Name]
	t1.name = t1Name
	t1.matchesPlayed++

	t2 := tally[t2Name]
	t2.name = t2Name
	t2.matchesPlayed++

	switch outcome {
	case "draw":
		t1.draws++
		t1.points++
		t2.draws++
		t2.points++
	case "loss":
		t1.losses++
		t2.wins++
		t2.points += 3
	case "win":
		t1.wins++
		t1.points += 3
		t2.losses++
	default:
		return fmt.Errorf("Undefined outcome %q", outcome)
	}
	tally[t1Name] = t1
	tally[t2Name] = t2
	return nil
}

type team struct {
	name                                       string
	matchesPlayed, wins, draws, losses, points int
}

func writeOutput(w io.Writer, tally map[string]team) {
	fmt.Fprintf(w, "Team                           | MP |  W |  D |  L |  P\n")
	teams := []team{}
	for _, team := range tally {
		teams = append(teams, team)
	}

	sort.Slice(teams, func(i, j int) bool {
		if teams[i].points != teams[j].points {
			return teams[i].points > teams[j].points
		}
		return teams[i].name < teams[j].name
	})
	for _, team := range teams {
		fmt.Fprintf(w, "%-31s| %2d | %2d | %2d | %2d | %2d\n", team.name, team.matchesPlayed, team.wins, team.draws, team.losses, team.points)
	}
}

// Tally tallies up the wins and losses in `r` and sends them to `w`
func Tally(r io.Reader, w io.Writer) error {
	scanner := bufio.NewScanner(r)
	tally := make(map[string]team)

	for scanner.Scan() {
		err := parseLine(scanner.Text(), tally)
		if err != nil {
			return err
		}
	}

	writeOutput(w, tally)
	return nil
}
