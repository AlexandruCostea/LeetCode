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
    ListNode* removeNthFromEnd(ListNode* head, int n) {
        if(head->next == nullptr)
            return nullptr;

        ListNode* aux = head;
        ListNode* deleteNode;
        int listSize = 0;
        while(aux != nullptr) {
            listSize++;
            aux = aux->next;
        }
        if(listSize < n+1) {
            deleteNode = head;
            head = head->next;
            delete deleteNode;
            return head;
        }
        aux = head;
        while(listSize != n+1) {
            listSize--;
            aux = aux->next;
        }
        deleteNode = aux->next;
        aux->next = aux->next->next;
        delete deleteNode;
        return head;
    }
};