pub const TEMPLATE_GET: &str = r###"    get:
      tags:
        - Users
      summary: Get all users
      operationId: listUsers
      parameters:
        - name: page
          in: query
          description: page
          required: false
          schema:
            type: integer
            format: int32
        - name: limit
          in: query
          description: limit
          required: false
          schema:
            type: integer
            format: int32
        - name: searchText
          in: query
          description: Only provide if you want to search with fullname/username/email
          required: false
          schema:
            type: string
      responses:
        "200":
          description: OK
          content:
            application/json:
              schema:
                allOf:
                  - $ref: "#/components/schemas/PageResult"
                  - type: object
                    properties:
                      items:
                        type: array
                        items:
                          $ref: "#/components/schemas/User"
        "401":
          description: Unauthorized
        "403":
          description: Forbidden
        "404":
          description: Not Found
      deprecated: false"###;

pub const TEMPLATE_POST: &str = r###"    post:
      tags:
        - Users
      summary: Create a user
      operationId: createUser
      requestBody:
        content:
          application/json:
            schema:
              type: object
              properties:
                fullname:
                  type: string
                username:
                  type: string
                email:
                  type: string
                password:
                  type: string
        description: input
        required: true
      responses:
        "200":
          description: OK
        "201":
          description: Created
        "401":
          description: Unauthorized
        "403":
          description: Forbidden
        "404":
          description: Not Found
      deprecated: false"###;

pub const TEMPLATE_PUT: &str = r###"    put:
      tags:
        - Users
      summary: Update user's status
      operationId: updateUserStatus
      parameters:
        - name: userId
          in: path
          description: userId
          required: true
          schema:
            type: string
      requestBody:
        content:
          application/json:
            schema:
              type: object
              properties:
                status:
                  $ref: "#/components/schemas/UserStatus"
        description: input
        required: true
      responses:
        "200":
          description: OK
        "401":
          description: Unauthorized
        "403":
          description: Forbidden
        "404":
          description: Not Found
      deprecated: false"###;

pub const TEMPLATE_DELETE: &str = r###"    delete:
      tags:
        - Users
      summary: Delete an user
      operationId: deleteUser
      parameters:
        - name: userId
          in: path
          description: userId
          required: true
          schema:
            type: string
      responses:
        "200":
          description: OK
        "401":
          description: Unauthorized
        "403":
          description: Forbidden
        "404":
          description: Not Found
      deprecated: false"###;
