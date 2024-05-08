# Porting Old Database to New Schema


> ##### 1. Identify What new Database will look like i.e. define it's schema:
>
> - User data like Id, Name, Email Id is required.
> - Type of transaction is required which will have only two values like 'Income' or 'Expense'
> - Transaction Date and Time is required.
> - Transaction Description is required.
> - For Now only 'Cash' transcations will be logged. In Future 'Debit Card' & 'Credit Card' will also be logged.
> - Lastly Transaction Amount is also required.
>
>
> ##### 2. Based On Above Criteria Tables Required can be as Follows:
>
> - User Table having column like 'id','name','email_id'
> - Transaction Type Table having columns like '_id','uuid','type','category'. Also 'category' will be having unique constraint.
> - Transactions/journal entry details table having columns like '_id','uuid', 'category (foreign key refrencing transaction type table)', 'amount', 'description', 'datetime'.
> - Transaction View will also be required which will list summary of transaction type and transcation detail.


# UI Dashbaord Data Requirements

> ##### 1. UI Dasboard Requires summary of entries done till date:
>
> - Income i.e all entries done in income entry type.
> - Expenses(same as above).
> - Balance of Income minux expenses.
> - Total no of entries done till date.
> - All categories summary for ex. food(total expense till date) and so on...
> - Lastly last 5 transaction upto previous day for example if today is 18-feb-2024 so dashboard require last 5 entries upto 17-feb-2024 in decreasing order(by date).