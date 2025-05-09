package p0002

type ListNode struct {
	Val  int
	Next *ListNode
}

func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	dummy := &ListNode{Val: 0}
	current := dummy
	carry := 0

	for l1 != nil || l2 != nil || carry > 0 {
		x := 0
		if l1 != nil {
			x = l1.Val
			l1 = l1.Next
		}
		y := 0
		if l2 != nil {
			y = l2.Val
			l2 = l2.Next
		}

		sum := x + y + carry
		carry = sum / 10

		current.Next = &ListNode{Val: sum % 10}
		current = current.Next
	}

	return dummy.Next
}
