#pragma once

#include <algorithm>
#include <vector>
#include <utility>
#include <mutex>
#include <functional>

template<typename T>
class ConcurrentVector {
    public:
        auto push_back(T&& item) -> void {
            const std::lock_guard<std::mutex> lock(m_mutex);
            m_data.push_back(std::forward<T>(item));
        }
        
        template<typename F>
        auto insert_sorted(T&& item, F&& predicate) {
            const std::lock_guard<std::mutex> lock(m_mutex);
            m_data.insert(std::upper_bound(m_data.begin(), m_data.end(), std::forward<T>(item), predicate), std::forward<T>(item));
        }
    
        template<typename F>
        auto push_back_if(T&& item, F&& predicate) -> bool {
            const std::lock_guard<std::mutex> lock(m_mutex);
            auto it = std::find_if(m_data.begin(), m_data.end(), predicate);
            if(it == m_data.end()) {
                m_data.push_back(std::forward<T>(item));
                return true;
            }
        
            return false;
        }
    
        template<typename F>
        auto find_and_erase(F&& predicate) -> void {
            const std::lock_guard<std::mutex> lock(m_mutex);
            std::remove_if(m_data.begin(), m_data.end(), predicate);
        }
    
        template<typename F>
        auto find(F&& predicate) -> T* {
            const std::lock_guard<std::mutex> lock(m_mutex);
            auto it = std::find_if(m_data.begin(), m_data.end(), predicate);
            return (it == m_data.end()) ? nullptr : std::addressof(*it);
        }
    
    private:
        std::mutex m_mutex;
        std::vector<T> m_data;
};