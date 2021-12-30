(ns core
  (:import GenerateNewAddress))

(defn -main
  [& args]
  (println (GenerateNewAddress/getNewAddress (first args))))
