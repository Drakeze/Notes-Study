// Java Enterprise - JDBC and JPA

import java.sql.*;
import javax.persistence.*;

/*
Topics covered:
1. JDBC (Java Database Connectivity)
2. JPA (Java Persistence API)
3. Transaction Management
4. Connection Pooling
*/

public class JDBC_JPA {
    public static void main(String[] args) {
        // TODO: Practice these concepts:

        // 1. JDBC
        // - Database connection
        // - Statement execution
        // - PreparedStatement
        // - ResultSet handling
        // - Transaction management

        // 2. JPA
        // - Entity mapping
        // - EntityManager
        // - JPQL
        // - Criteria API
        // - Relationships

        // 3. Transaction Management
        // - ACID properties
        // - Transaction isolation levels
        // - Commit and rollback
        // - Transaction boundaries

        // 4. Connection Pooling
        // - Connection pool configuration
        // - Pool sizing
        // - Connection lifecycle
        // - Connection validation
    }

    // JDBC example
    static class JDBCExample {
        private static final String URL = "jdbc:mysql://localhost:3306/dbname";
        private static final String USER = "username";
        private static final String PASSWORD = "password";

        public void basicJDBCOperations() {
            try (Connection conn = DriverManager.getConnection(URL, USER, PASSWORD)) {
                // Create statement
                Statement stmt = conn.createStatement();
                
                // Execute query
                ResultSet rs = stmt.executeQuery("SELECT * FROM users");
                
                // Process results
                while (rs.next()) {
                    // Handle result row
                }
                
                // Prepared statement
                PreparedStatement pstmt = conn.prepareStatement(
                    "INSERT INTO users (name, email) VALUES (?, ?)");
                pstmt.setString(1, "John Doe");
                pstmt.setString(2, "john@example.com");
                pstmt.executeUpdate();
                
            } catch (SQLException e) {
                e.printStackTrace();
            }
        }

        public void transactionExample() {
            Connection conn = null;
            try {
                conn = DriverManager.getConnection(URL, USER, PASSWORD);
                conn.setAutoCommit(false);
                
                // Perform multiple operations
                
                conn.commit();
            } catch (SQLException e) {
                try {
                    if (conn != null) conn.rollback();
                } catch (SQLException ex) {
                    ex.printStackTrace();
                }
                e.printStackTrace();
            }
        }
    }

    // JPA Entity example
    @Entity
    @Table(name = "users")
    static class User {
        @Id
        @GeneratedValue(strategy = GenerationType.IDENTITY)
        private Long id;

        @Column(name = "name")
        private String name;

        @Column(name = "email")
        private String email;

        // Getters and setters
    }

    // JPA operations example
    static class JPAExample {
        private EntityManagerFactory emf;
        private EntityManager em;

        public void init() {
            emf = Persistence.createEntityManagerFactory("persistence-unit");
            em = emf.createEntityManager();
        }

        public void basicJPAOperations() {
            // Create
            User user = new User();
            em.getTransaction().begin();
            em.persist(user);
            em.getTransaction().commit();

            // Read
            User foundUser = em.find(User.class, 1L);

            // Update
            em.getTransaction().begin();
            foundUser.setName("New Name");
            em.getTransaction().commit();

            // Delete
            em.getTransaction().begin();
            em.remove(foundUser);
            em.getTransaction().commit();

            // JPQL query
            TypedQuery<User> query = em.createQuery(
                "SELECT u FROM User u WHERE u.name = :name", User.class);
            query.setParameter("name", "John");
            List<User> users = query.getResultList();
        }

        public void cleanup() {
            em.close();
            emf.close();
        }
    }

    // Connection pool example (using HikariCP)
    static class ConnectionPoolExample {
        public void configureConnectionPool() {
            // Configuration example (pseudo-code)
            /*
            HikariConfig config = new HikariConfig();
            config.setJdbcUrl(URL);
            config.setUsername(USER);
            config.setPassword(PASSWORD);
            config.setMaximumPoolSize(10);
            
            HikariDataSource dataSource = new HikariDataSource(config);
            */
        }
    }
}