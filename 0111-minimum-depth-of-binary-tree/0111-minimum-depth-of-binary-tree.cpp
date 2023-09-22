/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    int minDepth(TreeNode* root) {
        if(root == nullptr)
            return 0;
        queue<TreeNode*> q;
        queue<int> dist;
        int minDist = INT32_MAX, distance;
        TreeNode* node;
        q.push(root);
        dist.push(1);
        while(!q.empty()) {
            node = q.front();
            distance = dist.front();
            q.pop();
            dist.pop();
            if(distance < minDist) {
                if(node->left == nullptr && node->right == nullptr) {
                    minDist = distance;
                }
                if(node->left != nullptr) {
                    q.push(node->left);
                    dist.push(distance+1);
                }
                if(node->right != nullptr) {
                    q.push(node->right);
                    dist.push(distance+1);
                }
            }
        }
        return minDist;
    }
};