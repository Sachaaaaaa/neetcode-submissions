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
    ListNode* reverseList(ListNode* head) {

        if (head == nullptr)  {
            return nullptr;
        }

        if (head->next == nullptr)  {
            return head;
        }

        ListNode* prev = nullptr; 
        ListNode* current = head; // A 
        ListNode* next = current->next; // B


        while (current != nullptr) {
            /*
            we set next here and not in the "Set the variable for next round" block
            because otherwise current could be null and thus we could dereference null with current->next
            */
            next = current->next;  

            current->next = prev; 

            // Set the variable for next round
            prev = current; 
            current = next; 
        }

        return prev;
    }
};
