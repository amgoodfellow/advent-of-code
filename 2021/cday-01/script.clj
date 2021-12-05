(ns day1
  (:require [clojure.string :as str]))

;; part one answer
(->> (slurp "test-input.txt")
     (str/split-lines)
     (map Integer/parseInt))




;; part two answer
(->> (slurp "input.txt")
     (str/split-lines)
     (map line->map)
     (filter #(has-valid-password? %))
     (count))
