struct ListNode* addTwoNumbers(struct ListNode* l1, struct ListNode* l2){
    struct ListNode* p = l1;
	struct ListNode* q = l2;
    struct ListNode* r;
	int x = 0,y;
	while(p || q){
		y = p->val+x;
		if(q){
			y+=q->val;
            if(!p->next && q->next){
			    p->next = (struct ListNode*)malloc(sizeof(struct ListNode));
			    p->next->val = 0;
			    p->next->next = NULL;
            }
            r = q;
			q=q->next;
            free(r);
		}
		x = y/10;
		y%=10;
		p->val = y;
        if(!p->next && x>0){
			p->next = (struct ListNode*)malloc(sizeof(struct ListNode));
			p->next->val = x;
            x = 0;
			p->next->next = NULL;
			break;
         }
		p = p->next;
	}
    return l1;
}