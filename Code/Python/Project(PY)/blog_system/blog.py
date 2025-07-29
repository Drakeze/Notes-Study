#This is a practice file for the blog system.
# It contains a simple class to represent a blog post.
#
from datetime import datetime
'''Add more attributes like:
◦  author
◦  date_created
◦  post_id
◦  tags/categories
•  Add more functionality like:
◦  editing posts
◦  deleting posts
◦  storing posts (database or file)
◦  method to format the post for different outputs (HTML, plain text)
◦  comment system
•  Add input validation
•  Add docstrings for better documentation'''
class BlogPost:
    # This is where the sign in and sign up functionality would be implemented.
    def __init__(self,author, title, content):
        self.author = author
        self.title = title
        self.content = content
        self.date_created = None
        self.post_id = None
        self.tags = []

#The titles and content are required parameters for creating a blog post.
    def set_date_created(self, date= None):
        if date is None:
            date = datetime.now()
        self.date_created = date
# Change of format for date_created for it to self add time when user made post 
    def set_post_id(self, post_id):
        self.post_id = post_id
    def add_tag(self, tag):
        if tag not in self.tags:
            self.tags.append(tag)
    def remove_tag(self, tag):
        if tag in self.tags:
            self.tags.remove(tag)
    def edit_content(self, new_content):
        self.content = new_content
    def edit_title(self, new_title):
        self.title = new_title
    def format_for_html(self):
        return f"<h1>{self.title}</h1><p>{self.content}</p>"
    def format_for_text(self):
        return f"Title: {self.title}\nContent: {self.content}"
    def __str__(self):
        return f"BlogPost(title={self.title}, author={self.author}, date_created={self.date_created}, post_id={self.post_id}, tags={self.tags})"
