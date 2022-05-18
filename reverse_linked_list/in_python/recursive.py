from typing import Optional

# Definition for singly-linked list.

class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def do_reverse(self, prev: Optional[ListNode], head: Optional[ListNode]):
        if head.next == None:
            head.next = prev
            return head
        if head.next != None:
            last = self.do_reverse(head, head.next)
        head.next = prev
        return last

    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None:
            return
        last = self.do_reverse(None, head)
        print(f"last: {last.val}")
        return last

    def printList(self, head: Optional[ListNode]) -> None:
        if head is None:
            return
        print(f"{head.val=}")
        if head.next:
            self.printList(head.next)

#nums = [1,2,3,4,5]
nums = []
prev = None

for i, n in enumerate(reversed(nums)):
    a = ListNode(n)
    if i != 0:
        a.next = prev
    prev = a

sol = Solution()
sol.printList(head=prev)
last = sol.reverseList(head=prev)
sol.printList(head=last)

