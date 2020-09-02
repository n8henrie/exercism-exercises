// Package account simulates a bank account
package account

import (
	"sync"
)

// Account represents an account
type Account struct {
	isClosed bool
	balance  int
	mu       sync.RWMutex
}

// Balance returns the account balance
func (a *Account) Balance() (int, bool) {
	a.mu.RLock()
	defer a.mu.RUnlock()

	if a.isClosed {
		return 0, false
	}
	return a.balance, true
}

// Open opens an account
func Open(amount int) *Account {
	if amount < 0 {
		return nil
	}
	return &Account{isClosed: false, balance: amount}
}

// Close closes the account
func (a *Account) Close() (int, bool) {
	a.mu.Lock()
	defer a.mu.Unlock()

	if a.isClosed {
		return 0, false
	}

	a.isClosed = true
	balance := a.balance
	a.balance = 0

	return balance, true
}

// Deposit adds funds to the account
func (a *Account) Deposit(amount int) (int, bool) {
	a.mu.Lock()
	defer a.mu.Unlock()

	endBalance := amount + a.balance
	if a.isClosed || (endBalance < 0) {
		return 0, false
	}
	a.balance = endBalance
	return a.balance, true
}
