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
    ListNode* reverseKGroup(ListNode* head, int k) {
        if(head == nullptr)
            return nullptr;
        deque<int> deq;
        ListNode* solHead = nullptr;
        ListNode* i;
        ListNode* aux;
        while(head) {
            deq.push_back(head->val);
            if(deq.size() == k) {
                while(deq.size() > 0) {
                    if(solHead == nullptr) {
                        solHead = new ListNode(deq.back());
                        i = solHead;
                    }
                    else {
                        i->next = new ListNode(deq.back());
                        i = i->next;
                    }
                    deq.pop_back();
                }
            }
            aux = head;
            head = head->next;
        }
        while(deq.size() > 0) {
            if(solHead == nullptr) {
                solHead = new ListNode(deq.front());
                i = solHead;
            }
            else {
                i->next = new ListNode(deq.front());
                i = i->next;
            }
            deq.pop_front();
        }
        return solHead;
    }
};