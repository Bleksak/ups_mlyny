#pragma once

#include <queue>
#include <mutex>

template<typename T>
class ConcurrentQueue {
    public:
        void push(T item) {
            const std::lock_guard<std::mutex> lock(m_mutex);
            m_queue.push(item);
        }
        
        T pop() {
            const std::lock_guard<std::mutex> lock(m_mutex);
            T item = m_queue.front();
            m_queue.pop();
            
            return item;
        }
        
        bool empty() {
            const std::lock_guard<std::mutex> lock(m_mutex);
            return m_queue.empty();
        }
        
    private:
        std::queue<T> m_queue;
        std::mutex m_mutex;
};
