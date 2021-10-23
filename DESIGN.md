# AskTheWorld project

## Database Entry Design

### Question

- id: UUID
- question: STRING
- createdAt: TIMESTAMP
- votesYes: INT
- votesNo: INT
- createdBy: STRING
- active: BOOL

## REST API Design:

### Route POST createQuestion
/api/createQuestion
Payload: JSON
- question: STRING
- createdBy: STRING

Return: JSON
- id: OBJECTID

### Route GET voteYes
/api/voteYes/{id}
Return: 200

### Route GET voteNo
/api/voteNo/{id}
Return: 200

### Route GET getResults
/api/getResults/{id}
Return: JSON
- question: STRING
- votesYes: INT
- votesNo: INT
- createdBy: STRING

### Route GET getQuestion
/api/getQuestion
Return: JSON
- id: OBJECTID
- question: STRING
- createdBy: STRING

