from typing import Optional
from dataclasses import dataclass


@dataclass
class TreeNode:
    val: int
    left = None
    right = None


def print_tree(root):
    queue = []

    queue.append(root)

    while len(queue) != 0:
        print(f"{queue[0].val}")
        node = queue.pop(0)

        if node.left is not None:
            queue.append(node.left)

        if node.right is not None:
            queue.append(node.right)


n1 = TreeNode(1)
n1.left = TreeNode(2)
n1.right = TreeNode(3)
n1.left.left = TreeNode(4)
n1.left.right = TreeNode(5)

print_tree(n1)

