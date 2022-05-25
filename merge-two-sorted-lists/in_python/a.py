from typing import Optional

# Definition for singly-linked list.

class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def mergeList(self, list1: Optional[ListNode], list2: Optional[ListNode]) -> Optional[ListNode]:
        cur = ListNode(-1)
        prehead = cur

        while list1 and list2:
            print(f"{cur.val=}")
            if list1.val <= list2.val:
                cur.next = list1
                list1 = list1.next
            else:
                cur.next = list2
                list2 = list2.next

            cur = cur.next

        if list1 is not None:
            cur.next = list1.next
        else:
            cur.next = list2.next

        return prehead.next

    def printList(self, head1: Optional[ListNode], head2: Optional[ListNode]) -> None:
        while head1 is not None:
            print(f"{head1.val=}")
            head1 = head1.next

        while head2 is not None:
            print(f"{head2.val=}")
            head2 = head2.next

nums1 = [1,2,3]
nums2 = [1,3,4]
# nums1 = []
# nums2 = [0]
prev1 = prev2 = None

for i, n in enumerate(reversed(nums1)):
    a = ListNode(n)
    if i != 0:
        a.next = prev1
    prev1 = a

for i, n in enumerate(reversed(nums2)):
    a = ListNode(n)
    if i != 0:
        a.next = prev2
    prev2 = a

sol = Solution()
sol.printList(head1=prev1, head2=prev2)
last = sol.mergeList(list1=prev1, list2=prev2)
sol.printList(head1=last, head2=None)

