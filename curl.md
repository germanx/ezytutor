# add course
curl -X POST localhost:3000/courses/ -H "Content-Type: application/json" -d '{"tutor_id":1, "course_name":"Hello , my first course !"}'

# add courses
curl -X POST localhost:3000/courses/ -H "Content-Type: application/json" -d '{"tutor_id":1, "course_name":"Hello , my first course !"}' 
curl -X POST localhost:3000/courses/ -H "Content-Type: application/json" -d '{"tutor_id":1, "course_name":"Hello , my second course !"}' 
curl -X POST localhost:3000/courses/ -H "Content-Type: application/json" -d '{"tutor_id":1, "course_name":"Hello , my third course !"}'


# github
https://github.com/peshwar9/rust-servers-services-apps