#pragma once
#include <functional>
#include <mutex>
#include <unordered_map>
#include <utility>

template<typename K, typename V>
class ConcurrentUnorderedMap {
    public:
        ConcurrentUnorderedMap<K, V> () {}
        
        void put(K& key, V&& value) {
            std::lock_guard<std::mutex> lock(m_mutex);
            m_map.insert(std::make_pair(key, std::move(value)));
        }
        
        V& operator[](K& key) {
            std::lock_guard<std::mutex> lock(m_mutex);
            return m_map[key];
        }
        
        void erase(K& key) {
            std::lock_guard<std::mutex> lock(m_mutex);
            m_map.erase(key);
        }
        
        template<typename T>
        T atomic_op(std::function<T (std::unordered_map<K, V>&)> callback) {
            std::lock_guard<std::mutex> lock(m_mutex);
            return callback(m_map);
        }
        
    private:
        std::unordered_map<K, V> m_map;
        std::mutex m_mutex;
};
