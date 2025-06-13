// Java Enterprise - Spring Framework Basics

import org.springframework.context.annotation.*;
import org.springframework.beans.factory.annotation.*;
import org.springframework.stereotype.*;

/*
Topics covered:
1. Spring Core
2. Dependency Injection
3. Spring MVC
4. Spring Boot
5. Spring Data
*/

// Note: This is a learning template. Actual Spring applications
// require proper project setup with Maven/Gradle

@Configuration
public class Spring_Framework {

    // 1. Spring Core and Dependency Injection

    // Bean definition
    @Bean
    public UserService userService() {
        return new UserService(userRepository());
    }

    @Bean
    public UserRepository userRepository() {
        return new UserRepository();
    }

    // Component example
    @Component
    static class UserComponent {
        private final UserService userService;

        @Autowired
        public UserComponent(UserService userService) {
            this.userService = userService;
        }
    }

    // Service layer example
    @Service
    static class UserService {
        private final UserRepository userRepository;

        public UserService(UserRepository userRepository) {
            this.userRepository = userRepository;
        }

        public User findById(Long id) {
            return userRepository.findById(id);
        }
    }

    // Repository layer example
    @Repository
    static class UserRepository {
        public User findById(Long id) {
            // Database access logic
            return null;
        }
    }

    // 2. Spring MVC

    // Controller example
    @Controller
    @RequestMapping("/api/users")
    static class UserController {
        private final UserService userService;

        @Autowired
        public UserController(UserService userService) {
            this.userService = userService;
        }

        @GetMapping("/{id}")
        public ResponseEntity<User> getUser(@PathVariable Long id) {
            User user = userService.findById(id);
            return ResponseEntity.ok(user);
        }

        @PostMapping
        public ResponseEntity<User> createUser(@RequestBody User user) {
            // Create user logic
            return ResponseEntity.ok(user);
        }
    }

    // 3. Spring Boot Application

    /*
    @SpringBootApplication
    public class DemoApplication {
        public static void main(String[] args) {
            SpringApplication.run(DemoApplication.class, args);
        }
    }
    */

    // 4. Spring Data JPA

    // Entity example
    @Entity
    static class User {
        @Id
        @GeneratedValue(strategy = GenerationType.IDENTITY)
        private Long id;

        private String username;
        private String email;

        // Getters and setters
    }

    // JPA Repository interface
    /*
    public interface UserJpaRepository extends JpaRepository<User, Long> {
        List<User> findByUsername(String username);
        
        @Query("SELECT u FROM User u WHERE u.email = ?1")
        User findByEmail(String email);
    }
    */

    // 5. Spring Security

    /*
    @Configuration
    @EnableWebSecurity
    public class SecurityConfig extends WebSecurityConfigurerAdapter {
        
        @Override
        protected void configure(HttpSecurity http) throws Exception {
            http
                .authorizeRequests()
                    .antMatchers("/", "/home").permitAll()
                    .anyRequest().authenticated()
                    .and()
                .formLogin()
                    .loginPage("/login")
                    .permitAll()
                    .and()
                .logout()
                    .permitAll();
        }
    }
    */

    // 6. Spring AOP

    @Aspect
    @Component
    static class LoggingAspect {
        @Before("execution(* com.example.service.*.*(..))"))
        public void logBefore(JoinPoint joinPoint) {
            // Logging logic
        }

        @Around("@annotation(Transactional)")
        public Object transactionAdvice(ProceedingJoinPoint joinPoint) throws Throwable {
            // Transaction handling logic
            return joinPoint.proceed();
        }
    }

    // 7. Spring Configuration Properties

    @ConfigurationProperties(prefix = "app")
    static class AppProperties {
        private String name;
        private String description;

        // Getters and setters
    }

    // 8. Spring RestTemplate

    @Component
    static class RestClientExample {
        private final RestTemplate restTemplate;

        public RestClientExample(RestTemplate restTemplate) {
            this.restTemplate = restTemplate;
        }

        public User getUser(Long id) {
            return restTemplate.getForObject(
                "http://api.example.com/users/{id}",
                User.class,
                id
            );
        }
    }
}