/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Solution {
public:
    vector<int> distanceK(TreeNode* root, TreeNode* target, int k) {
        if(root == nullptr)
            return vector<int>{};
        unordered_map<int, vector<int>> graph;
        queue<TreeNode*> q;
        if(root->left != nullptr) {
            q.push(root->left);
            graph[root->val].push_back(root->left->val);
            graph[root->left->val].push_back(root->val);
        }
        if(root->right != nullptr) {
            q.push(root->right);
            graph[root->val].push_back(root->right->val);
            graph[root->right->val].push_back(root->val);
        }
        while(!q.empty()) {
            TreeNode* node = q.front();
            q.pop();
            if(node->left != nullptr) {
                q.push(node->left);
                graph[node->val].push_back(node->left->val);
                graph[node->left->val].push_back(node->val);
            }
            if(node->right != nullptr) {
                q.push(node->right);
                graph[node->val].push_back(node->right->val);
                graph[node->right->val].push_back(node->val);
            }
        }
        queue<int> bfsQueue;
        queue<int> dist;
        vector<int> sol;
        set<int> visited;
        bfsQueue.push(target->val);
        dist.push(0);
        while(!bfsQueue.empty()) {
            int node = bfsQueue.front();
            int distance = dist.front();
            bfsQueue.pop();
            dist.pop();
            visited.insert(node);
            if(distance < k) {
                for(int i = 0; i < graph[node].size(); i++) {
                    if(visited.find(graph[node][i]) == visited.end()) {
                        bfsQueue.push(graph[node][i]);
                        dist.push(distance+1);
                    }
                }
            }
            else if(distance == k)
                sol.push_back(node);
        }
        return sol;
    }
};