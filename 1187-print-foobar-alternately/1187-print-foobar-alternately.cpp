class FooBar {
private:
    int n;

public:
    mutex fooM,barM;
    FooBar(int n) {
        this->n = n;
        barM.lock();
    }

    void foo(function<void()> printFoo) {
        
        for (int i = 0; i < n; i++) {
            
        	// printFoo() outputs "foo". Do not change or remove this line.
            fooM.lock();
        	printFoo();
            barM.unlock();
        }
    }

    void bar(function<void()> printBar) {
        
        for (int i = 0; i < n; i++) {
            
        	// printBar() outputs "bar". Do not change or remove this line.
            barM.lock();
        	printBar();
            fooM.unlock();
        }
    }
};