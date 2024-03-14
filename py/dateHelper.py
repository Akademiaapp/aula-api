from datetime import datetime, timedelta, timezone

def get_current_week_dates(formated = True):
    # Get current date
    today = datetime.now().replace(hour=0, minute=0, second=0, microsecond=0)
    
    # Calculate the start of the week (Monday)
    start_of_week = today - timedelta(days=today.weekday())
    
    # Calculate the end of the week (Sunday)
    end_of_week = start_of_week + timedelta(days=5)
    
    # Add timezone information
    tz_info = timezone(timedelta(hours=1))  # Adjust as per your timezone
    start_of_week = start_of_week.replace(tzinfo=tz_info)
    end_of_week = end_of_week.replace(hour=23, minute=59, second=59, microsecond=999999, tzinfo=tz_info)
    
    if(formated):
        return format(start_of_week), format(end_of_week)
    else:
        return start_of_week, end_of_week

def format(date):
    return date.strftime('%Y-%m-%d %H:%M:%S.%f%z')
    
start_date, end_date = get_current_week_dates()
print("start:", start_date)
print("end:", end_date)