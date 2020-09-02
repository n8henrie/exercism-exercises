// Package tree creates a tree from some records
package tree

import (
	"errors"
	"sort"
)

// Record has an ID and a Parent
type Record struct {
	ID     int
	Parent int
}

// Node has an ID and a slice of references to child nodes
type Node struct {
	ID       int
	Children []*Node
}

func getOrCreateNodeByID(tree map[int]*Node, id int) *Node {
	var node *Node

	if n, ok := tree[id]; ok {
		// Node exists
		node = n

	} else {
		// Node doesn't exist yet, create and insert it
		node = &Node{
			ID: id,
		}
		tree[id] = node
	}
	return node
}

// Build creates a tree out of records
func Build(records []Record) (*Node, error) {
	var root *Node
	maxID := -1
	tree := make(map[int]*Node, len(records))

	sort.Slice(records, func(i, j int) bool { return records[i].ID < records[j].ID })
	for _, record := range records {
		if record.ID > maxID {
			maxID = record.ID
		}
		node := getOrCreateNodeByID(tree, record.ID)
		if record.ID == record.Parent {
			if root != nil {
				return nil, errors.New("Duplicate root")
			}
			root = node
		} else {

			parent := getOrCreateNodeByID(tree, record.Parent)
			if node.ID <= parent.ID {
				return nil, errors.New("Child ID must be higher than parent")
			}

			for _, child := range parent.Children {
				if child.ID == node.ID {
					return nil, errors.New("Duplicate node")
				}
			}
			parent.Children = append(parent.Children, node)
		}
	}

	if maxID != len(records)-1 {
		return nil, errors.New("Couldn't create a proper tree out of records")
	}

	return root, nil
}
