(defun standardize (in)
  (if 
      (equal "X" in)
      "A"
    (if 
	(equal "Y" in)
	"B"
      (if 
	  (equal  "Z" in)
	  "C"
	)))
  )

(defun standardize-line (in)
  (aset in 2 (string-to-char
	      (standardize (char-to-string
			    (aref in 2)))))
  in
  )

(defun get-points (in)
  (let ((moves '(("A" . 1)
		 ("B" . 2)
		 ("C" . 3))))
    (cdr (assoc in moves))))

(defun outcome (me elf)
  (let ((beats '(("A" . "C")
		 ("B" . "A")
		 ("C" . "B"))))
    (if (equal (cdr (assoc me beats)) elf)
	"win"
      (if (equal me elf)
	  "draw"
	"lose"))))

(defun add_to_score (me elf)
  (if (equal "draw" (outcome me elf))
	     (progn
	       (setq elf_score (+ (get-points elf) (+ 3 elf_score)))
	       (setq my_score (+ (get-points me) (+ 3 my_score))))
	     (if (equal "win" (outcome me elf))
			(progn
			  (setq elf_score (+ (get-points elf) elf_score))
			  (setq my_score (+ (get-points me) (+ 6 my_score))))
			(progn
			  (setq elf_score (+ (get-points elf) (+ 6 elf_score)))
			  (setq my_score (+ (get-points me) my_score))))))
	       
(defvar elf_score 0)
(defvar my_score 0)
     
(defun process_line (line)
  (if (equal (length line) 3)
      (add_to_score (char-to-string (aref line 0)) (char-to-string (aref (standardize-line line) 2)))))

(let (source-code (current-buffer))
  (switch-to-buffer "input.txt")

  (let ((contents (buffer-substring-no-properties 1 (point-max))))
     (mapc 'process_line (split-string contents "\n")))

  (switch-to-buffer source-code))

(print elf_score)
(print my_score)
