class Student
{
    private fullName: string;

    constructor(firstName: string, lastName: string)
    {
        this.fullName = firstName + " " + lastName;
    }

    getFullName(): string
    {
        return this.fullName;
    }
};

function greeter(student: Student): string
{
    return "Hello, " + student.getFullName();
}

var user: Student = new Student("Jane", "User");

console.log(greeter(user));
