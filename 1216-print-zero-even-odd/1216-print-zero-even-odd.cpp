class ZeroEvenOdd {
private:
    int n,start;
    mutex mOdd,mEven,mZero;

public:
    ZeroEvenOdd(int n) {
        this->n = n;
        this->start = 1;
        mEven.lock();
        mOdd.lock();
    }

    void zero(function<void(int)> printNumber) {
        while(this->start <= this->n) {
            mZero.lock();
            if(this->start <= this->n) {
                printNumber(0);
                if(this->start%2 == 1)
                    mOdd.unlock();
                else
                    mEven.unlock();
            }
        }
        mOdd.unlock();
        mEven.unlock();
    }

    void even(function<void(int)> printNumber) {
        while(this->start <= this->n) {
            mEven.lock();
            if(this->start <= this->n) {
                printNumber(this->start);
                this->start++; mZero.unlock();
            }
        }
    }

    void odd(function<void(int)> printNumber) {
            while(this->start <= this->n) {
            mOdd.lock();
            if(this->start <= this->n) {
                printNumber(this->start);
                this->start++; mZero.unlock();
            }
        }
    }
};