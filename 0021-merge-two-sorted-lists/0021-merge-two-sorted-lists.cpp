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
    ListNode* mergeTwoLists(ListNode* list1, ListNode* list2) {
        ListNode* mergedList = nullptr;
        ListNode* result = nullptr;
        ListNode* aux;
        if(list1!=nullptr && list2!=nullptr &&(list1->val < list2->val) ||
        (list1!=nullptr && list2 == nullptr)) {
            mergedList = new ListNode(list1->val);
            aux = list1;
            list1 = list1->next;
            delete aux;
        }
        else if (list2!=nullptr){
            mergedList = new ListNode(list2->val);
            aux = list2;
            list2 = list2->next;
            delete aux;
        }
        result = mergedList;
        while(list1!=nullptr && list2!=nullptr) {
            if(list1->val < list2->val) {
                mergedList->next = new ListNode(list1->val);
                aux = list1;
                list1 = list1->next;
                delete aux;
                mergedList = mergedList->next;
            }
            else {
                mergedList->next = new ListNode(list2->val);
                aux = list2;
                list2 = list2->next;
                delete aux;
                mergedList = mergedList->next;
            }
        }
        while(list1!=nullptr) {
            mergedList->next = new ListNode(list1->val);
            aux = list1;
            list1 = list1->next;
            delete aux;
            mergedList = mergedList->next;
        }
        while(list2!=nullptr) {
            mergedList->next = new ListNode(list2->val);
            aux = list2;
            list2 = list2->next;
            delete aux;
            mergedList = mergedList->next;
        }
        return result;
    }
};
