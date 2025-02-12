;; This Source Code Form is subject to the terms of the Mozilla Public
;; License, v. 2.0. If a copy of the MPL was not distributed with this
;; file, You can obtain one at http://mozilla.org/MPL/2.0/.
;;
;; Copyright (c) KALEIDOS INC

(ns app.common.logic.variants
  (:require
   [app.common.files.changes-builder :as pcb]
   [cuerdas.core :as str]))

(defn properties-to-name
  [properties]
  (->> properties
       (map :value)
       (str/join ", ")))

(defn generate-add-new-property
  ([changes related-components]
   (let [property-name (str "Property" (-> related-components
                                           first
                                           :variant-properties
                                           count
                                           inc))]
     (generate-add-new-property changes related-components property-name)))

  ([changes related-components property-name]
   (let [[_ changes]
         (reduce (fn [[num changes] component]
                   (let [main-id      (:main-instance-id component)
                         _ (prn "Adding" property-name (str "Value" num))
                         add-variant #(let [_ (prn %)
                                            _ (prn "add-variant " property-name (str "Value" num))]
                                        (-> (or % [])
                                            (conj {:name property-name :value (str "Value" num)})))
                         add-name #(let [_ (prn %)]
                                     (if (str/empty? %)
                                       (str "Value" num)
                                       (str % ", " "Value" num)))]
                     [(inc num)
                      (-> changes
                          (pcb/update-component (:id component) #(update % :variant-properties add-variant))
                          (pcb/update-shapes [main-id] #(update % :variant-name add-name)))]))
                 [1 changes]
                 related-components)]
     changes)))