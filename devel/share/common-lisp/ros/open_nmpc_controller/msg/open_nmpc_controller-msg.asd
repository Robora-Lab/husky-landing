
(cl:in-package :asdf)

(defsystem "open_nmpc_controller-msg"
  :depends-on (:roslisp-msg-protocol :roslisp-utils :std_msgs-msg
)
  :components ((:file "_package")
    (:file "OptimizationParameters" :depends-on ("_package_OptimizationParameters"))
    (:file "_package_OptimizationParameters" :depends-on ("_package"))
    (:file "OptimizationResult" :depends-on ("_package_OptimizationResult"))
    (:file "_package_OptimizationResult" :depends-on ("_package"))
    (:file "Status" :depends-on ("_package_Status"))
    (:file "_package_Status" :depends-on ("_package"))
  ))