#pragma once

#include <queue>
#include <mutex>
#include <iostream>

template<typename T>
class ConcurrentQueue {
    public:
        auto push(T item) -> void {
            const std::lock_guard<std::mutex> lock(m_mutex);
            m_queue.push(std::move(item));
        }
        
        auto pop() -> T {
            const std::lock_guard<std::mutex> lock(m_mutex);
            T item = std::move(m_queue.front());
            m_queue.pop();
            
            return item;
        }
        
        auto empty() -> bool {
            const std::lock_guard<std::mutex> lock(m_mutex);
            return m_queue.empty();
        }
        
    private:
        std::queue<T> m_queue;
        std::mutex m_mutex;
};
