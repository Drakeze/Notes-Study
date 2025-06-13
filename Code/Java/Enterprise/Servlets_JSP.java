// Java Enterprise - Servlets and JSP

import javax.servlet.*;
import javax.servlet.http.*;
import java.io.*;

/*
Topics covered:
1. Servlets
2. JSP (JavaServer Pages)
3. Filters and Listeners
4. Session Management
5. Web Application Structure
*/

public class Servlets_JSP {
    // Note: This is a learning template. Actual servlet applications
    // need to be deployed in a web container like Tomcat

    // 1. Basic Servlet Example
    public class BasicServlet extends HttpServlet {
        @Override
        protected void doGet(HttpServletRequest request, 
                           HttpServletResponse response) 
                           throws ServletException, IOException {
            // Set response content type
            response.setContentType("text/html");
            
            // Get writer
            PrintWriter out = response.getWriter();
            
            // Write response
            out.println("<html><body>");
            out.println("<h1>Hello from Servlet!</h1>");
            out.println("</body></html>");
        }

        @Override
        protected void doPost(HttpServletRequest request, 
                            HttpServletResponse response) 
                            throws ServletException, IOException {
            // Handle POST requests
            String username = request.getParameter("username");
            // Process form data
        }
    }

    // 2. Filter Example
    public class AuthenticationFilter implements Filter {
        @Override
        public void init(FilterConfig filterConfig) throws ServletException {
            // Initialization code
        }

        @Override
        public void doFilter(ServletRequest request, 
                           ServletResponse response, 
                           FilterChain chain) 
                           throws IOException, ServletException {
            HttpServletRequest httpRequest = (HttpServletRequest) request;
            
            // Check authentication
            if (isAuthenticated(httpRequest)) {
                chain.doFilter(request, response);
            } else {
                // Redirect to login page
            }
        }

        @Override
        public void destroy() {
            // Cleanup code
        }

        private boolean isAuthenticated(HttpServletRequest request) {
            // Authentication logic
            return false;
        }
    }

    // 3. Listener Example
    public class AppContextListener implements ServletContextListener {
        @Override
        public void contextInitialized(ServletContextEvent sce) {
            // Application startup code
        }

        @Override
        public void contextDestroyed(ServletContextEvent sce) {
            // Application shutdown code
        }
    }

    // 4. Session Management Example
    public class SessionServlet extends HttpServlet {
        @Override
        protected void doGet(HttpServletRequest request, 
                           HttpServletResponse response) 
                           throws ServletException, IOException {
            // Get or create session
            HttpSession session = request.getSession();
            
            // Set session attribute
            session.setAttribute("user", "John Doe");
            
            // Get session attribute
            String user = (String) session.getAttribute("user");
            
            // Session management
            session.setMaxInactiveInterval(1800); // 30 minutes
            
            // Remove session attribute
            session.removeAttribute("user");
            
            // Invalidate session
            session.invalidate();
        }
    }

    // 5. Cookie Management Example
    public class CookieServlet extends HttpServlet {
        @Override
        protected void doGet(HttpServletRequest request, 
                           HttpServletResponse response) 
                           throws ServletException, IOException {
            // Create cookie
            Cookie cookie = new Cookie("username", "johndoe");
            cookie.setMaxAge(24 * 60 * 60); // 24 hours
            response.addCookie(cookie);
            
            // Read cookies
            Cookie[] cookies = request.getCookies();
            if (cookies != null) {
                for (Cookie c : cookies) {
                    if ("username".equals(c.getName())) {
                        String username = c.getValue();
                        // Use username
                    }
                }
            }
        }
    }

    /*
    // JSP Example (typically in .jsp file)
    <%@ page language="java" contentType="text/html; charset=UTF-8" pageEncoding="UTF-8"%>
    <!DOCTYPE html>
    <html>
    <head>
        <title>JSP Example</title>
    </head>
    <body>
        <%-- Declaration --%>
        <%! int count = 0; %>
        
        <%-- Scriptlet --%>
        <% 
            count++;
            String username = request.getParameter("username");
        %>
        
        <%-- Expression --%>
        <h1>Welcome, <%= username %></h1>
        <p>Visit count: <%= count %></p>
        
        <%-- Using JavaBeans --%>
        <jsp:useBean id="user" class="com.example.User" scope="session">
            <jsp:setProperty name="user" property="name" value="John Doe"/>
        </jsp:useBean>
        
        <%-- Including other files --%>
        <%@ include file="header.jsp" %>
        
        <%-- Custom tags --%>
        <%@ taglib uri="http://java.sun.com/jsp/jstl/core" prefix="c" %>
        <c:if test="${not empty username}">
            <p>Logged in as: ${username}</p>
        </c:if>
    </body>
    </html>
    */
}