/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    ListNode* mergeKLists(vector<ListNode*>& lists) {
        priority_queue<int, vector<int>, greater<int>> orderedElements;
        for(int i = 0; i < lists.size(); i++) {
            while(lists[i] != nullptr) {
                orderedElements.push(lists[i]->val);
                lists[i] = lists[i]->next;
            }
        }
        if(orderedElements.empty())
            return nullptr;
        ListNode* result = new ListNode(orderedElements.top());
        orderedElements.pop();
        ListNode* finalList = result;
        while(!orderedElements.empty()) {
            finalList->next = new ListNode(orderedElements.top());
            finalList = finalList->next;
            orderedElements.pop();
        }
        finalList->next = nullptr;
        return result;
    }
};