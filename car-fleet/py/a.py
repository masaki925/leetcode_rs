from typing import List


class Solution:
    def carFleet(self, target: int, position: List[int], speed: List[int]) -> int:
        pair = [(p, s) for p, s in zip(position, speed)]
        pair.sort(reverse=True)
        stack = []
        for p, s in pair:  # Reverse Sorted Order
            stack.append((target - p) / s)
            if len(stack) >= 2 and stack[-1] <= stack[-2]:
                stack.pop()
        print(stack)
        return len(stack)


target = 10
position = [8, 3, 7, 4, 6, 5]
speed = [4, 4, 4, 4, 4, 4]
print(Solution().carFleet(target, position, speed))
