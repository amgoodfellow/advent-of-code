(ns day1
  (:require [clojure.string :as str]))

;; part one answer
(->> (slurp "input.txt")
     (str/split-lines)
     (map #(Integer/parseInt %))
     (partition 2 1)
     (filter #(< (first %) (last %)))
     (count))

;; part two answer
(->> (slurp "input.txt")
     (str/split-lines)
     (map #(Integer/parseInt %))
     (partition 3 1)
     (map #(reduce + %))
     (partition 2 1)
     (filter #(< (first %) (last %)))
     (count))
